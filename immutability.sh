#!/usr/bin/env bash
if [ "$1" == "check" ]
then
	if [ "$(lsattr / | grep usr | cut -c 5-5)" == "i" ]
	then
		echo "File System is set to: Read-Only"
	else
		echo "File System is set to: Read-Write"
	fi
	exit
fi

if [ "$(whoami)" != "root" ]
then
	echo "Error: You must run this script as root."
	exit
fi

if [ "$1" == "set" ]
then
	if [ "$2" == "ro" ]
	then
		chattr +i /
		chattr +i -R /usr
		echo "File System is now: Read-Only"
	elif [ "$2" == "rw" ]
	then
		chattr -i /
		chattr -i -R /usr
		echo "File System is now: Read-Write"
	fi
fi

if [ "$1" != "check" ]
then
	echo "Error: "$1" not understood."
fi
