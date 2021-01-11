package api

import (
	"regexp"
	"strings"
)

func parseInfo(info string) map[string]*string {
	lines := strings.Split(info, "\n")

	parsed := make(map[string]*string)
	for _, line := range lines {
		re := regexp.MustCompile(`\* ([^:]+): (.+)`)
		matches := re.FindStringSubmatch(line)
		if matches != nil {
			if matches[2] == "None" {
				parsed[matches[1]] = nil
			} else {
				parsed[matches[1]] = &matches[2]
			}
		}
	}
	return parsed
}
