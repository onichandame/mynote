package model

import (
	"github.com/google/uuid"
	goutils "github.com/onichandame/go-utils"
	"gorm.io/gorm"
)

type Universal struct {
	UUID string `gorm:"not null;unique"`
}

func (n *Universal) BeforeCreate(tx *gorm.DB) (err error) {
	defer goutils.RecoverToErr(&err)
	n.UUID = uuid.NewString()
	return err
}
