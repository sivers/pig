package main

import (
	"database/sql"
	"fmt"
	"os"
	"strings"

	"github.com/golang-migrate/migrate/v4"
	"github.com/golang-migrate/migrate/v4/database/postgres"

	// Go migrator: source driver import
	_ "github.com/golang-migrate/migrate/v4/source/file"
	// Go Postgres driver for database/sql
	_ "github.com/lib/pq"
	"github.com/pkg/errors"
	"github.com/sirupsen/logrus"
)

var dbInstance *sql.DB
var schema *string

// SetSchema set schema name
func SetSchema(name string) {
	schema = &name
}

func migrateInternal() error {

	dbInstance := GetInstance()
	// defer dbInstance.Close()

	postgresConfig := &postgres.Config{}
	driver, err := postgres.WithInstance(dbInstance, postgresConfig)
	if err != nil {
		return errors.Wrap(err, "problem getting driver")
	}

	path, err := os.Getwd()
	if err != nil {
		return err
	}

	migrator, err := migrate.NewWithDatabaseInstance(
		fmt.Sprintf("file://%s", path),
		postgresConfig.DatabaseName,
		driver)
	if err != nil {
		return errors.Wrap(err, "problem getting migrator")
	}

	logrus.Info("running db migration")

	err = migrator.Up()
	if err != nil {
		if err == migrate.ErrNoChange {
			logrus.Warn("no changes in migration")
		} else {
			return errors.Wrapf(err, "problem running migration")
		}
	}

	return nil
}

// GetInstance get postgres instance
func GetInstance() *sql.DB {
	if dbInstance == nil {
		var err error
		dbInstance, err = sql.Open("postgres", fmt.Sprintf("host=%s port=%s dbname=%s user=%s password=%s sslmode=%s", "localhost", "2222", "pig", "pig", "pig", "disable"))
		if err != nil {
			panic("problem getting db instance: " + err.Error())
		}
	}

	return dbInstance
}

// Q Execute query for psql function
func Q(fName string, params []string) (*int, string, error) {
	var query string
	if len(params) < 1 {
		query = fmt.Sprintf("SELECT status, js FROM %s.%s()", *schema, fName)
	} else {
		query = fmt.Sprintf("SELECT status, js FROM %s.%s('%s')", *schema, fName, strings.Join(params, "','"))
	}

	var statusCode int
	var js string

	err := GetInstance().QueryRow(query).Scan(&statusCode, &js)
	if err != nil && err != sql.ErrNoRows {
		return nil, "", fmt.Errorf("failed to execute sql query row funcName: %s params %v", fName, params)
	}

	return &statusCode, js, nil
}
