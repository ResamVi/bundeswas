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

func main() {
	store, err := NewStore()
	if err != nil {
		fmt.Println("could not start program:", err)
		os.Exit(1)
	}

	p := tea.NewProgram(model{
		spinner:  spinner.New(),
		progress: progress.New(),

		store:      store,
		start:      time.Now(),
		totalCount: 1, // Avoid division by zero
		dashboard:  make([]string, 3),
		downloads:  make(chan dip.PlenarprotokollText),
	})

	if _, err := p.Run(); err != nil {
		fmt.Println("could not start program:", err)
		os.Exit(1)
	}
}

type model struct {
	progress progress.Model
	spinner  spinner.Model

	start        time.Time
	store        *Store
	downloads    chan dip.PlenarprotokollText
	currentCount int
	totalCount   int
	percent      float64
	done         bool
	dashboard    []string
}

func (m model) Init() tea.Cmd {
	return tea.Sequence(
		// Prepare
		prepareDownload(),

		// Download
		tea.Batch(
			m.spinner.Tick,
			downloadPlenarprotokolle(m.downloads),
			waitForMore(m.downloads, m.store),
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
		return m, waitForMore(m.downloads, m.store)

	// Listen for newly downloaded Plenarprotokolle and count up progress.
	case downloadMsg:
		m.currentCount++
		m.dashboard = append(m.dashboard[1:], msg.id)

		if m.currentCount == m.totalCount {
			m.done = true
			m.store.Close()
			return m, tea.Quit
		}
		return m, waitForMore(m.downloads, m.store) // wait for next event

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

	for _, id := range m.dashboard {
		if id == "" {
			progress += pad + "........................\n"
		} else {
			progress += pad + fmt.Sprintf("Protokoll %s downloaded\n", id)
		}
	}

	progress += "\n" + pad + helpStyle("Press any key to quit")

	return s + progress
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
func waitForMore(downloads chan dip.PlenarprotokollText, store *Store) tea.Cmd {
	return func() tea.Msg {
		document := <-downloads
		store.Insert(document)
		return downloadMsg{id: document.Id}
	}
}

// Storer implements logic to write Plenarprotokolle to a file.
func NewStore() (*Store, error) {
	f, err := os.Create("./protokolle.txt")
	if err != nil {
		return nil, fmt.Errorf("could not create file for storing")
	}

	return &Store{file: f}, nil
}

type Store struct {
	file *os.File
}

func (s Store) Insert(protokoll dip.PlenarprotokollText) error {
	if protokoll.Text == nil {
		return fmt.Errorf("Plenarprotokoll without text found: %+v", protokoll)
	}

	_, err := s.file.WriteString(*protokoll.Text + "\n\n\n")
	if err != nil {
		return fmt.Errorf("could not write to file: %w", err)
	}

	return nil
}

func (s Store) Close() error {
	return s.file.Close()
}
