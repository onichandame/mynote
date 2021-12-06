package main

import (
	"github.com/onichandame/gim"
	"github.com/onichandame/mynote/server"
)

func main() {
	root := gim.Module{Imports: []*gim.Module{&server.ServerModule}}
	root.Bootstrap()
	server := root.Get(new(server.Server)).(*server.Server)
	server.Run()
}
