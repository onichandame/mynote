package auth

import (
	"fmt"
	"time"

	"github.com/golang-jwt/jwt"
	"github.com/google/uuid"
	goutils "github.com/onichandame/go-utils"
	"github.com/onichandame/mynote/model"
	"gorm.io/gorm"
)

type sessionService struct {
	db *gorm.DB
}

func newSessionsService(db *gorm.DB) *sessionService {
	var svc sessionService
	svc.db = db
	return &svc
}

func (svc *sessionService) create(user *model.User) string {
	key := svc.getKey()
	tok := jwt.NewWithClaims(jwt.SigningMethodHS256, jwt.StandardClaims{
		Subject: fmt.Sprintf("%v", user.ID),
	})
	tok.Header["kid"] = key.ID
	res, err := tok.SignedString(key.Key)
	goutils.Assert(err)
	return res
}

func (svc *sessionService) getKey() *model.SessionKey {
	var keyCount int64
	goutils.Assert(svc.db.Model(new(model.SessionKey)).Count(&keyCount).Error)
	if keyCount < 1 {
		goutils.Assert(svc.db.Create(&model.SessionKey{Key: uuid.NewString()}).Error)
	} else {
		var key model.SessionKey
		goutils.Assert(svc.db.Order("created_at desc").First(&key).Error)
		if time.Since(key.CreatedAt) > time.Hour*24*31 {
			goutils.Assert(svc.db.Create(&model.SessionKey{Key: uuid.NewString()}).Error)
		}
	}
	var key model.SessionKey
	goutils.Assert(svc.db.Order("created_at desc").First(&key).Error)
	return &key
}
