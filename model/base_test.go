package model_test

import (
	"testing"

	"github.com/onichandame/mynote/model"
	"github.com/stretchr/testify/assert"
	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
)

func TestUniversal(t *testing.T) {
	type Ent struct {
		gorm.Model
		model.Universal
	}
	db, err := gorm.Open(sqlite.Open(`:memory:`))
	assert.Nil(t, err)
	assert.NotNil(t, db)
	assert.Nil(t, db.AutoMigrate(new(Ent)))
	t.Run("hook", func(t *testing.T) {
		var ent Ent
		assert.Nil(t, db.Create(&ent).Error)
		assert.NotEmpty(t, ent.UUID)
	})
}
