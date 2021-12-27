package store

import (
	"encoding/json"
	"io"
	"os"
)

type Repository[T any] struct {
	file *os.File
}

func (db *Repository[T]) Store(item T) error {
	read, err := io.ReadAll(db.file)
	if err != nil {
		panic(err)
	}

	var list []T
	err = json.Unmarshal(read, &list)
	if len(read) != 0 && err != nil {
		panic(err)
	}

	list = append(list, item)

	write, err := json.Marshal(list)
	if err != nil {
		return err
	}

	_, err = db.file.Write(write)
	if err != nil {
		return err
	}

	return nil
}

func New[T any]() Repository[T] {
	f, err := os.OpenFile("db.txt", os.O_RDWR|os.O_APPEND|os.O_CREATE,0644)
	if err != nil {
		panic(err)
	}

	return Repository[T]{
		file: f,
	}
}
