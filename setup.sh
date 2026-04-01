#!/bin/sh

RED='\033[0;31m'
YELLOW='\033[0;33m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
LIGHT_GRAY='\033[0;33m'

print() {
  if [ ! -n "$2" ]; then
    print_error "NO ARGUMENT PROVIDED TO PRINT FOR ERROR"
   exit 2
  fi

  color=$2

  if [ ! -n "$1" ]; then
    print_error "NO ARGUMENT PROVIDED TO PRINT_INFO"
    exit 1
  fi

  if [ ! -n "$3" ]; then
    print_error "NO ARGUMENT PROVIDED FOR MESSAGE TYPE"
    exit 3
  fi

  text=$1
  message_type=$3

  printf "%s %s: $text %s\n" "$color" "$message_type" "$NO_COLOR"
}

print_error() {
  print "$1" "$RED" "ERROR"
}

print_warning() {
  print "$1" "$YELLOW" "WARNING"
}

print_info() {
  print "$1" "$CYAN" "INFO"
}

print_debug() {
  print "$1" "$LIGHT_GRAY" "DEBUG"
}

print_good() {
    print "$1" "$GREEN" "OK"
}

check_package() {
    if [ ! -n "$1" ]; then
        print_error "NO ARGUMENT PROVIDED TO CHECK PACKAGE"
        exit 2
    fi

    package="$1"
    print_info "Checking if $package is installed and on PATH...\n"
    if command -v "$package" >/dev/null 2>&1; then
        print_good "$package" "is found continuning ...\n"
    else
        print_error "$package not is found"
        print_warning "Install $package with either apt/dnf/pacman/brew/ports"
        print_warning "    (any other way as long as we check it on the PATH)"
        exit 1
    fi
    
}

check_package "python"
check_package "clang"
./setup.py
