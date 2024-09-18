package config

import (
	"fmt"
	"os"
	"runtime"

	"github.com/jedib0t/go-pretty/v6/table"
)

/* --------------------------------------------------------------------------- */
/*       DO NOT MOVE THESE VARIABLES ANYWHERE ELSE OR THE BUILD WILL FAIL      */
/* --------------------------------------------------------------------------- */

// These values should be treated as constants generated during compile time.
var (
	// LibraryName returns the name of the binary.
	LibraryName string
	// GitCommit returns the git commit that was compiled. This will be filled in by the compiler.
	GitCommit string
	// Version returns the main version number that is being run at the moment.
	Version string
	// BuildDate returns the date the binary was built.
	BuildDate string
	// GoVersion returns the version of the go runtime used to compile the binary.
	GoVersion = runtime.Version()
	// OsArch returns the os and arch used to build the binary.
	OsArch = fmt.Sprintf("%s/%s", runtime.GOOS, runtime.GOARCH)
)

// PrintVersionInfo prints information about the current version of the software.
func PrintVersionInfo() {
	t := table.NewWriter()
	t.SetOutputMirror(os.Stdout)
	t.SetStyle(table.StyleColoredBright)
	t.AppendHeader(table.Row{"Info", "Value"})
	t.AppendRows([]table.Row{
		{"Library Name", LibraryName},
		{"Build Date", BuildDate},
		{"Git Commit", GitCommit},
		{"Version", Version},
		{"Go Version", GoVersion},
		{"OS/Arch", OsArch},
	})
	t.AppendSeparator()
	t.Render()
}
