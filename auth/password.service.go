package auth

import (
	goutils "github.com/onichandame/go-utils"
	"golang.org/x/crypto/bcrypt"
)

type passwordService struct{}

func newPasswordService() *passwordService {
	var svc passwordService
	return &svc
}

func (svc *passwordService) hash(raw string) string {
	enc, err := bcrypt.GenerateFromPassword([]byte(raw), bcrypt.DefaultCost)
	goutils.Assert(err)
	return string(enc)
}

func (svc *passwordService) validate(raw string, hashed string) bool {
	return bcrypt.CompareHashAndPassword([]byte(hashed), []byte(raw)) == nil
}
