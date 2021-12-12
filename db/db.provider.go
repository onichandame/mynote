package db

import (
	goutils "github.com/onichandame/go-utils"
	"github.com/onichandame/mynote/model"
	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
)

func newDB() *gorm.DB {
	db, err := gorm.Open(sqlite.Open(":memory:"))
	goutils.Assert(err)
	goutils.Assert(db.AutoMigrate(new(model.User), new(model.SessionKey), new(model.Note)))
	return db
}
