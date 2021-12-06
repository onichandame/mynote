package ui

import (
	"io/fs"
	"net/http"

	"github.com/gin-gonic/gin"
	gimgin "github.com/onichandame/gim-gin"
)

type uiController struct{}

func newUIController(dir fs.FS, ginsvc *gimgin.GinService) *uiController {
	var ctl uiController
	ginsvc.AddRoute(func(rg *gin.RouterGroup) {
		rg.StaticFS("dashboard", http.FS(dir))
	})
	return &ctl
}
