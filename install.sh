#!/usr/bin/bash

INSTALL_DIR=/usr/bin/
ASSETS_DIR=/usr/share/goats

if [[ !(-e assets/xyz.jacobmealey.Notes.desktop) ]]
then
    echo "no assets directory, are you sure you're in the right spot?"
    exit 1
fi

if [[ `whoami` != 'root' ]] 
then
    echo "You must be root to run this script..."
    exit 1
fi

mkdir -p $INSTALL_DIR
mkdir -p $ASSETS_DIR
cp assets/clean_notes_db.sql $ASSETS_DIR/notes_db.sql
cp assets/xyz.jacobmealey.Notes.desktop /usr/share/applications/xyz.jacobmealey.Notes.desktop
# cp for eventual icon
cp target/debug/notes $INSTALL_DIR/notes

