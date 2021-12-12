package server

import (
	"context"
	"reflect"

	"github.com/gin-gonic/gin"
	gimgin "github.com/onichandame/gim-gin"
	gimgraphql "github.com/onichandame/gim-graphql"
	goutils "github.com/onichandame/go-utils"
	gqlwsmessage "github.com/onichandame/gql-ws/message"
	gqlwsserver "github.com/onichandame/gql-ws/server"
	"github.com/onichandame/mynote/auth"
	"github.com/onichandame/mynote/model"
)

type controller struct{}

func newController(gqlsvc *gimgraphql.GraphqlService, ginsvc *gimgin.GinService, sessvc *auth.SessionService) *controller {
	var ctl controller
	schema := gqlsvc.BuildSchema()
	ginsvc.AddRoute(func(rg *gin.RouterGroup) {
		rg.GET("graphql", func(c *gin.Context) {
			var user *model.User
			gqlwsserver.NewSocket(&gqlwsserver.Config{
				Schema:   schema,
				Response: c.Writer,
				Request:  c.Request,
				OnConnectionInit: func(m *gqlwsmessage.Message) gqlwsmessage.Payload {
					defer goutils.RecoverToErr(new(error))
					if m, ok := m.Payload.(map[string]interface{}); ok {
						if session, ok := m[`session`]; ok {
							if sess, ok := session.(string); ok {
								user = sessvc.Validate(sess)
							}
						}
					}
					return nil
				},
				Context: context.WithValue(context.Background(), reflect.TypeOf(new(model.User)), user),
			})
		})
	})
	return &ctl
}
