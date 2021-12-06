package server

import (
	"context"
	"fmt"
	"net/http"
	"os"
	"os/signal"
	"syscall"

	"github.com/gin-gonic/gin"
	gimgin "github.com/onichandame/gim-gin"
)

type Server struct {
	engine *gin.Engine
}

func newServer(ginsvc *gimgin.GinService) *Server {
	var srv Server
	srv.engine = ginsvc.Bootstrap()
	return &srv
}

func (s *Server) Run() {
	srv := http.Server{
		Handler: s.engine,
		Addr:    fmt.Sprintf("0.0.0.0:80"),
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
