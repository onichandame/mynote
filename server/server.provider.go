package server

import (
	"context"
	"fmt"
	"net/http"
	"os"
	"os/signal"
	"syscall"

	gimgin "github.com/onichandame/gim-gin"
)

type Server struct {
	ginsvc *gimgin.GinService
}

func newServer(ginsvc *gimgin.GinService) *Server {
	var srv Server
	srv.ginsvc = ginsvc
	return &srv
}

func (s *Server) Run() {
	port := os.Getenv(`PORT`)
	if port == "" {
		port = `80`
	}
	srv := http.Server{
		Handler: s.ginsvc.Bootstrap(),
		Addr:    fmt.Sprintf("0.0.0.0:%v", port),
	}
	done := make(chan int)
	sigchan := make(chan os.Signal)
	signal.Notify(sigchan, os.Interrupt, syscall.SIGTERM)
	go func() {
		<-sigchan
		srv.Shutdown(context.TODO())
		close(done)
	}()
	go func() { srv.ListenAndServe() }()
	<-done
}
