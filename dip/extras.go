package dip

import (
	"context"
	"fmt"
	"net/http"
)

// The codegen does not create these types so we declare them manually.
type (
	BadRequestResponseCode      int
	NotFoundResponseCode        int
	UnauthorizedResponseCode    int
	UnauthorizedResponseMessage string
)

// The codegen does not come with authentication implemented so we add it to the client.
func Authenticate(key string) ClientOption {
	return func(c *Client) error {
		// This function will be called before every request.
		fn := func(ctx context.Context, req *http.Request) error {
			fmt.Println(key)
			req.Header.Set("Authorization", "ApiKey "+key)
			return nil
		}

		c.RequestEditors = append(c.RequestEditors, fn)
		return nil
	}
}
