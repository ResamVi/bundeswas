package main

import (
	"fmt"
	"os"
	"strings"
	"time"

	"github.com/charmbracelet/bubbles/progress"
	"github.com/charmbracelet/bubbles/spinner"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
	"github.com/resamvi/bundeswas/dip"
)

type model struct {
	progress progress.Model
	spinner  spinner.Model

	start        time.Time
	downloaded   chan dip.PlenarprotokollText
	currentCount int
	totalCount   int
	percent      float64
	done         bool
	results      []string
}

type prepareMsg struct{ count int }

func prepareDownload() tea.Cmd {
	return func() tea.Msg {
		count, err := dip.NewClient().GetCount()
		if err != nil {
			panic(err) // TODO: Return
		}

		return prepareMsg{count: count}
	}
}

// A command that starts downloadign Plenarprotokolle and forwards them to a channel.
func downloadPlenarprotokolle(downloads chan dip.PlenarprotokollText) tea.Cmd {
	return func() tea.Msg {
		stream := dip.NewClient().DownloadProtokolle()
		for document := range stream {
			downloads <- document
		}

		return tea.Quit
	}
}

// Indicate that new Plenarprotokolle were downloaded.
type downloadMsg struct{ id string }

// A command that listens for downloaded Plenarprotokolle and sends a message to render an update.
func waitForMore(downloads chan dip.PlenarprotokollText) tea.Cmd {
	return func() tea.Msg {
		d := <-downloads
		return downloadMsg{id: d.Id}
	}
}

func (m model) Init() tea.Cmd {
	return tea.Sequence(
		// Prepare
		prepareDownload(),

		// Download
		tea.Batch(
			m.spinner.Tick,
			downloadPlenarprotokolle(m.downloaded),
			waitForMore(m.downloaded),
		),
	)
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	const maxWidth = 80

	switch msg := msg.(type) {

	// Listen for any keypresses then quit.
	case tea.KeyMsg:
		return m, tea.Quit

	// Listen for answer to how many documents exist ("preparation").
	case prepareMsg:
		m.totalCount = msg.count
		return m, waitForMore(m.downloaded)

	// Listen for newly downloaded Plenarprotokolle and count up progress.
	case downloadMsg:
		m.currentCount++
		m.results = append(m.results[1:], msg.id)

		if m.currentCount == m.totalCount {
			m.done = true
			return m, tea.Quit
		}
		return m, waitForMore(m.downloaded) // wait for next event

	// Listen for when window is resized.
	case tea.WindowSizeMsg:
		m.progress.Width = msg.Width - 4
		if m.progress.Width > maxWidth {
			m.progress.Width = maxWidth
		}
		return m, nil

	// Listen for events that should render the next animation frame for spinner.
	case spinner.TickMsg:
		var cmd tea.Cmd
		m.spinner, cmd = m.spinner.Update(msg)
		return m, cmd

	default:
		return m, nil
	}
}

var (
	helpStyle = lipgloss.NewStyle().Foreground(lipgloss.Color("#626262")).Render
	doneStyle = lipgloss.NewStyle().Margin(1, 2).Render
)

func (m model) View() string {
	const padding = 2 // space between left border and progress bar

	if m.done {
		return doneStyle(fmt.Sprintf("Done! Downloaded %d Plenarprotokolle in %s.\n", m.currentCount, time.Since(m.start)))
	}

	s := fmt.Sprintf("\n %s (%d / %d) Plenarprotokolle downloaded: \n", m.spinner.View(), m.currentCount, m.totalCount)
	percentage := float64(m.currentCount) / float64(m.totalCount)

	pad := strings.Repeat(" ", padding)
	progress := "\n" +
		pad + m.progress.ViewAs(percentage) + "\n\n"

	for _, id := range m.results {
		if id == "" {
			progress += pad + "........................\n"
		} else {
			progress += pad + fmt.Sprintf("Protokoll %s downloaded\n", id)
		}
	}

	progress += "\n" + pad + helpStyle("Press any key to quit")

	return s + progress
}

func main() {
	p := tea.NewProgram(model{
		spinner:  spinner.New(),
		progress: progress.New(),

		start:      time.Now(),
		totalCount: 1, // Avoid division by zero
		results:    make([]string, 3),
		downloaded: make(chan dip.PlenarprotokollText),
	})

	if _, err := p.Run(); err != nil {
		fmt.Println("could not start program:", err)
		os.Exit(1)
	}
}
