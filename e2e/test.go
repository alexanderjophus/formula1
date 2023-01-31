package main

import (
	"fmt"

	playwright "github.com/playwright-community/playwright-go"
)

func main() {
	if err := run(); err != nil {
		panic(err)
	}
}

func run() error {
	err := playwright.Install()
	if err != nil {
		return fmt.Errorf("installing playwright: %w", err)
	}
	pw, err := playwright.Run()
	if err != nil {
		return fmt.Errorf("could not start playwright: %v", err)
	}
	browser, err := pw.Chromium.Launch()
	if err != nil {
		return fmt.Errorf("could not launch browser: %v", err)
	}
	page, err := browser.NewPage()
	if err != nil {
		return fmt.Errorf("could not create page: %v", err)
	}
	page.SetDefaultTimeout(5 * 1000)
	if _, err = page.Goto("localhost:8081"); err != nil {
		return fmt.Errorf("could not goto: %v", err)
	}
	if err := myTest(page); err != nil {
		_, errScreenshot := page.Screenshot(playwright.PageScreenshotOptions{
			Path: playwright.String("screenshot.png"),
		})
		if errScreenshot != nil {
			return fmt.Errorf("could not take screenshot: %v", errScreenshot)
		}
		return fmt.Errorf("test failed: %v", err)
	}
	if err = browser.Close(); err != nil {
		return fmt.Errorf("could not close browser: %v", err)
	}
	if err = pw.Stop(); err != nil {
		return fmt.Errorf("could not stop Playwright: %v", err)
	}
	return nil
}

func myTest(page playwright.Page) error {
	if err := checkDriversPage(page); err != nil {
		return fmt.Errorf("could not check Drivers page: %v", err)
	}
	if err := checkConstructorsPage(page); err != nil {
		return fmt.Errorf("could not check Constructors page: %v", err)
	}
	if err := checkSchedulePage(page); err != nil {
		return fmt.Errorf("could not check Schedule page: %v", err)
	}
	return nil
}

func checkDriversPage(page playwright.Page) error {
	if err := page.Click("text=Drivers"); err != nil {
		return fmt.Errorf("could not click Drivers: %v", err)
	}
	// check that the page is loaded
	ele, err := page.WaitForSelector("text=Drivers Standings")
	if err != nil {
		return fmt.Errorf("could not wait for Drivers: %v", err)
	}
	if ele == nil {
		return fmt.Errorf("could not find Drivers")
	}
	return nil
}

func checkConstructorsPage(page playwright.Page) error {
	if err := page.Click("text=Constructors"); err != nil {
		return fmt.Errorf("could not click Constructors: %v", err)
	}
	// check that the page is loaded
	ele, err := page.WaitForSelector("text=Constructors Standings")
	if err != nil {
		return fmt.Errorf("could not wait for Constructors: %v", err)
	}
	if ele == nil {
		return fmt.Errorf("could not find Constructors")
	}
	return nil
}

func checkSchedulePage(page playwright.Page) error {
	if err := page.Click("text=Schedule"); err != nil {
		return fmt.Errorf("could not click Schedule: %v", err)
	}
	// check that the page is loaded
	ele, err := page.WaitForSelector("text=Schedule")
	if err != nil {
		return fmt.Errorf("could not wait for Schedule: %v", err)
	}
	if ele == nil {
		return fmt.Errorf("could not find Schedule")
	}
	return nil
}
