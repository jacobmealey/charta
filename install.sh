#!/usr/bin/bash



INSTALL_DIR=$1/usr/bin/
ASSETS_DIR=$1/usr/share/charta
echo "Installing to " $1

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
mkdir -p $ASSETS_DIR/json/
mkdir -p $1/usr/share/applications
mkdir -p $1/usr/share/pixmaps
cp assets/xyz.jacobmealey.Notes.desktop $1/usr/share/applications/xyz.jacobmealey.Notes.desktop
cp assets/starter.txt $ASSETS_DIR/json/starter.txt
cp assets/style.css $ASSETS_DIR/style.css
cp assets/bitmap.png $1/usr/share/pixmaps/charta.png
# cp for eventual icon
cp target/release/notes $INSTALL_DIR/notes

chmod -R o+w $ASSETS_DIR

