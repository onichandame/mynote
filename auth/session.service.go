package auth

import (
	"errors"
	"fmt"
	"time"

	"github.com/golang-jwt/jwt"
	"github.com/google/uuid"
	goutils "github.com/onichandame/go-utils"
	"github.com/onichandame/mynote/model"
	"gorm.io/gorm"
)

type SessionService struct {
	db *gorm.DB
}

func newSessionsService(db *gorm.DB) *SessionService {
	var svc SessionService
	svc.db = db
	return &svc
}

func (svc *SessionService) Validate(sess string) *model.User {
	token, err := jwt.Parse(sess, func(t *jwt.Token) (res interface{}, err error) {
		defer goutils.RecoverToErr(&err)
		if _, ok := t.Method.(*jwt.SigningMethodHMAC); !ok {
			panic(errors.New(`signing method incorrect`))
		}
		if kid, ok := t.Header[`kid`]; ok {
			var key model.SessionKey
			goutils.Assert(svc.db.First(&key, kid).Error)
			res = []byte(key.Key)
		} else {
			panic(errors.New(`kid not present`))
		}
		return res, err
	})
	goutils.Assert(err)
	if claims, ok := token.Claims.(*jwt.StandardClaims); ok {
		var user model.User
		goutils.Assert(svc.db.First(&user, claims.Subject).Error)
		return &user
	} else {
		panic(errors.New(`session claim invalid`))
	}
}

func (svc *SessionService) create(user *model.User) string {
	key := svc.getKey()
	tok := jwt.NewWithClaims(jwt.SigningMethodHS256, jwt.StandardClaims{
		Subject:   fmt.Sprintf("%v", user.ID),
		ExpiresAt: time.Now().Add(time.Hour * 24).Unix(),
	})
	tok.Header["kid"] = key.ID
	res, err := tok.SignedString(key.Key)
	goutils.Assert(err)
	return res
}
func (svc *SessionService) getKey() *model.SessionKey {
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
