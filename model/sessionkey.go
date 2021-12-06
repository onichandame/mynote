package model

import "gorm.io/gorm"

type SessionKey struct {
	gorm.Model
	Key string `gorm:"not null;unique"`
}
