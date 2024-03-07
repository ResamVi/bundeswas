package dip

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"strings"
)

// Client which conforms to the OpenAPI3 specification for this service.
type Client struct {
	// The endpoint of the server serving the HTTP interface.
	URL    string
	Client *http.Client

	// key is the API key for authorization.
	// See: https://dip.bundestag.de/%C3%BCber-dip/hilfe/api#content
	key string
}

// Creates a new Client with known default values.
func NewClient() *Client {
	// API key changes routinely so this code may break in the future.
	// See: https://dip.bundestag.de/%C3%BCber-dip/hilfe/api#content
	const (
		key = "rgsaY4U.oZRQKUHdJhF9qguHMkwCGIoLaqEcaHjYLF"
		url = "https://search.dip.bundestag.de/api/v1"
	)

	client := Client{
		URL:    url,
		Client: &http.Client{},
		key:    key,
	}

	// ensure the server URL always has a trailing slash.
	if !strings.HasSuffix(client.URL, "/") {
		client.URL += "/"
	}

	return &client
}

// WithKey overrides the default API key.
func (c *Client) WithKey(key string) *Client {
	c.key = key
	return c
}

func (c *Client) DownloadProtokolle() chan PlenarprotokollText {
	downloads := make(chan PlenarprotokollText, 100)

	go func() {
		cursor := ""
		for {
			resp, err := c.GetProtokolle(cursor)
			if err != nil {
				panic(err) // TODO:
			}
			for _, document := range resp.Documents {
				downloads <- document
			}

			if len(resp.Documents) == 0 {
				close(downloads)
				break
			}

			cursor = resp.Cursor
		}
	}()

	return downloads
}

// getCount returns the total amount of Plenarprotokolle available.
func (c *Client) GetCount() (int, error) {
	req, err := http.NewRequest("GET", c.URL+"/plenarprotokoll-text", nil)
	if err != nil {
		return 0, fmt.Errorf("could not create request to get count: %w", err)
	}

	resp, err := c.doRequest(req)
	if err != nil {
		return 0, fmt.Errorf("could not do request to get count: %w", err)
	}

	return resp.NumFound, nil
}

// GetProtokolle gets a list of plenarprotokolle with their full text.
func (c *Client) GetProtokolle(cursor ...string) (*Response, error) {
	requestURL, err := url.Parse(c.URL + "/plenarprotokoll-text")
	if err != nil {
		return nil, err
	}

	// If a cursor was given use that in the request.
	if len(cursor) == 1 && cursor[0] != "" {
		values := requestURL.Query()
		values.Add("cursor", cursor[0])

		requestURL.RawQuery = values.Encode()
	}

	req, err := http.NewRequest("GET", requestURL.String(), nil)
	if err != nil {
		return nil, err
	}
	req.Header.Add("Authorization", "ApiKey "+c.key)

	rsp, err := c.Client.Do(req)
	if err != nil {
		return nil, err
	}

	bodyBytes, err := io.ReadAll(rsp.Body)
	if err != nil {
		return nil, err
	}
	defer rsp.Body.Close()

	switch {
	case rsp.StatusCode == 200:
		var dest Response
		if err := json.Unmarshal(bodyBytes, &dest); err != nil {
			return nil, err
		}
		return &dest, nil

	case rsp.StatusCode == 400 || rsp.StatusCode == 401 || rsp.StatusCode == 404:
		var dest ErrorResponse
		if err := json.Unmarshal(bodyBytes, &dest); err != nil {
			return nil, err
		}
		return nil, fmt.Errorf("error status returned: %w", dest)

	default:
		return nil, errors.New("unknown status code: " + rsp.Status)
	}
}

// doRequest sends an HTTP request.
//
// It contains all implementation details for communicating with the API.
func (c *Client) doRequest(req *http.Request) (*Response, error) {
	req.Header.Add("Authorization", "ApiKey "+c.key)

	rsp, err := c.Client.Do(req)
	if err != nil {
		return nil, fmt.Errorf("could not do request: %w", err)
	}

	bodyBytes, err := io.ReadAll(rsp.Body)
	if err != nil {
		return nil, fmt.Errorf("could not read response body: %w", err)
	}
	defer rsp.Body.Close()

	switch {
	case rsp.StatusCode == 200:
		var dest Response
		if err := json.Unmarshal(bodyBytes, &dest); err != nil {
			return nil, fmt.Errorf("could not unmarshal response: %w\n%s", err, string(bodyBytes))
		}
		return &dest, nil

	case rsp.StatusCode == 400 || rsp.StatusCode == 401 || rsp.StatusCode == 404:
		if bytes.Contains(bodyBytes, []byte("Rate-Limit exceeded.")) {
			return nil, errors.New("rate limit exceeded")
		}

		var dest ErrorResponse
		if err := json.Unmarshal(bodyBytes, &dest); err != nil {
			return nil, fmt.Errorf("could not unmarshal error response: %w\n%s", err, string(bodyBytes))
		}
		return nil, fmt.Errorf("error status returned: %w", dest)

	default:
		return nil, errors.New("unknown status code: " + rsp.Status)
	}
}
