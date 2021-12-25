package database

import (
	"encoding/json"
	"os"
)

type Db struct {
	file *os.File
}

func (db *Db) Store(val interface{}) error {

	bytes, err := json.Marshal(val)
	if err != nil {
		return err
	}

	_, err = db.file.Write(bytes)
	if err != nil {
		return err
	}

	return nil
}

func New() Db {
	f, err := os.OpenFile("db.txt", os.O_CREATE|os.O_APPEND|os.O_WRONLY, 0644)
	if err != nil {
		panic(err)
	}

	return Db{
		file: f,
	}
}
