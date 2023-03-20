#!/bin/bash

export PGPASSWORD='test'; 
psql -h localhost -d postgres -U postgres -p 5432 -w -a -q -f setup/db/db.sql
