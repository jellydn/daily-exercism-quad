package techpalace

import "strings"

// WelcomeMessage returns a welcome message for the customer.
func WelcomeMessage(customer string) string {
	upperCustomer := strings.ToUpper(customer)
	return "Welcome to the Tech Palace, " + upperCustomer
}

// AddBorder adds a border to a welcome message.
func AddBorder(welcomeMsg string, numStarsPerLine int) string {
	stars := strings.Repeat("*", numStarsPerLine)
	return stars + "\n" + welcomeMsg + "\n" + stars
}

// CleanupMessage cleans up an old marketing message.
func CleanupMessage(oldMsg string) string {
	lines := strings.Split(oldMsg, "\n")
	if len(lines) < 3 {
		return ""
	}

	content := lines[1]
	content = strings.Trim(content, "*")
	content = strings.TrimSpace(content)

	return content
}
