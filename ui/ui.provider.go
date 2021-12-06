package ui

import (
	"embed"
	"io/fs"

	goutils "github.com/onichandame/go-utils"
)

//go:generate yarn build

//go:embed dist/*
var root embed.FS

func newUI() fs.FS {
	dist, err := fs.Sub(root, "dist")
	goutils.Assert(err)
	return dist
}
