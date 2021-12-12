package auth

import (
	"context"
	"errors"
	"reflect"

	"github.com/onichandame/mynote/model"
	"gorm.io/gorm"
)

func Self(ctx context.Context) func(*gorm.DB) *gorm.DB {
	user, ok := ctx.Value(reflect.TypeOf(new(model.User))).(*model.User)
	if !ok || user == nil {
		panic(errors.New(`self not found`))
	}
	return func(d *gorm.DB) *gorm.DB {
		return d.Where("user_id = ?", user.ID)
	}
}
