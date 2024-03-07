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
	percent float64

	progress progress.Model
	spinner  spinner.Model

	downloaded   chan []dip.PlenarprotokollText
	currentCount int
	totalCount   int
}

type tickMsg time.Time

// Bubble Tea will run this asynchronously.
func downloadPlenarprotokolle(downloads chan []dip.PlenarprotokollText) tea.Cmd {
	return func() tea.Msg {
		cursor := ""
		client := dip.NewClient()

		for {
			resp, err := client.GetProtokolle(cursor)
			if err != nil {
				panic(err) // TODO
			}
			cursor = resp.Cursor

			if len(resp.Documents) == 0 {
				break
			}

			downloads <- resp.Documents
		}

		return tea.Quit // TODO: Success message
	}
}

// Indicate how many new Plenarprotokolle were downloaded.
type downloadMsg struct{ count int }

// A command that waits for downloaded Plenarprotokolle and sends a message to report how many.
func waitForMore(downloads chan []dip.PlenarprotokollText) tea.Cmd {
	return func() tea.Msg {
		return downloadMsg{count: len(<-downloads)}
	}
}

func (m model) Init() tea.Cmd {
	// return tea.Sequence(
	// 	tea.Batch(
	// 		tea.Println("A"),
	// 		tea.Println("B"),
	// 		tea.Println("C"),
	// 	),
	// 	tea.Println("Z"),
	// 	tea.Quit,
	// )

	return tea.Batch(
		m.spinner.Tick,
		downloadPlenarprotokolle(m.downloaded), // generate activity
		waitForMore(m.downloaded),              // wait for activity
	)
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	const maxWidth = 80

	switch msg := msg.(type) {

	// Listen for any keypresses then quit.
	case tea.KeyMsg:
		return m, tea.Quit

	// Listen for newly downloaded Plenarprotokolle and count up progress.
	case downloadMsg:
		m.currentCount += msg.count
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

var helpStyle = lipgloss.NewStyle().Foreground(lipgloss.Color("#626262")).Render

func (m model) View() string {
	const padding = 2 // space between left border and progress bar

	s := fmt.Sprintf("\n %s (%d / %d) Plenarprotokolle downloaded: \n", m.spinner.View(), m.currentCount, m.totalCount)
	percentage := float64(m.currentCount) / float64(m.totalCount)

	pad := strings.Repeat(" ", padding)
	progress := "\n" +
		pad + m.progress.ViewAs(percentage) + "\n\n" +
		pad + helpStyle("Press any key to quit")

	return s + progress
}

func main() {
	count, err := dip.NewClient().GetProtokollCount()
	if err != nil {
		panic(err) // TODO
	}

	p := tea.NewProgram(model{
		totalCount: count,
		downloaded: make(chan []dip.PlenarprotokollText),
		spinner:    spinner.New(),
		progress:   progress.New(),
	})

	if _, err := p.Run(); err != nil {
		fmt.Println("could not start program:", err)
		os.Exit(1)
	}

}
