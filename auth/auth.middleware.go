package auth

import (
	"errors"
	"strings"

	"github.com/gin-gonic/gin"
	"github.com/golang-jwt/jwt"
	gimgin "github.com/onichandame/gim-gin"
	goutils "github.com/onichandame/go-utils"
	"github.com/onichandame/mynote/model"
	"gorm.io/gorm"
)

type authMiddleware struct{}

func newAuthMiddleware(db *gorm.DB, ginsvc *gimgin.GinService) *authMiddleware {
	var mw authMiddleware
	ginsvc.AddMiddleware(func(c *gin.Context) {
		header := c.GetHeader("Authorization")
		splitTok := strings.Split(header, "Bearer")
		if len(splitTok) > 1 {
			token := strings.TrimSpace(splitTok[1])
			tok, err := jwt.ParseWithClaims(token, new(jwt.StandardClaims), func(t *jwt.Token) (res interface{}, err error) {
				defer goutils.RecoverToErr(&err)
				kid, ok := t.Header["kid"]
				if !ok {
					panic(errors.New("jwt does not have valid kid"))
				}
				var key model.SessionKey
				goutils.Assert(db.First(&key, kid).Error)
				res = key.Key
				return res, err
			})
			if err == nil {
				claim := tok.Claims.(*jwt.StandardClaims)
				var user model.User
				goutils.Assert(db.First(&user, claim.Subject).Error)
				c.Set("user", &user)
			}
		}
		c.Next()
	})
	return &mw
}
