#!/bin/sh

RED='\033[0;31m'
YELLOW='\033[0;33m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
LIGHT_GRAY='\033[0;33m'
NO_COLOR='\033[0m'

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

print_info "CHECKING IF THE MESON BUILD TOOL IS INSTALLED"
if ! command -v meson >/dev/null 2>&1; then
  print_info "MESON BUILD SYSTEM IS NOT INSTALLED"
  print_info "ATTEMPTING TO INSTALL MESON"
  print_info "CHECKING FOR A WORKING PIP COMMAND"
  PIP_CMD=""
  if command -v pip3 >/dev/null 2>&1; then
    PIP_CMD="pip3"
  elif command -v python3 >/dev/null 2>&1 && python3 -m pip --version >/dev/null 2>&1; then
    PIP_CMD="python3 -m pip"
  elif command -v pip >/dev/null 2>&1; then
    PIP_CMD="pip"
  elif command -v python >/dev/null 2>&1 && python -m pip --version >/dev/null 2>&1; then
    PIP_CMD="python -m pip"
  fi

  if [ -z "$PIP_CMD" ]; then
    print_error "NO WORKING PIP COMMAND FOUND"
    print_error "TRIED: pip3, python3 -m pip, pip, python -m pip"
    print_error "ABORTING ALL"
    exit 10
  fi

  print_good "FOUND PIP AS: $PIP_CMD"
  print_info "INSTALLING MESON THROUGH PIP"
  $PIP_CMD install --user meson

  if command -v meson >/dev/null 2>&1; then
    print_good "MESON IS INSTALLED SUCCESSFULLY"
  else
    print_warning "MESON WAS INSTALLED BUT IS NOT ON PATH"
    print_warning "ADD YOUR PYTHON USER SCRIPTS DIRECTORY TO PATH, REOPEN THE TERMINAL, AND RUN setup.sh AGAIN"
    exit 11
  fi
else
  print_good "MESON IS INSTALLED ON THE SYSTEM"
fi

print_debug "CALCULATING PATH TO SUBPROJECTS SUB-DIRECTORY..."
subprojects_directory="$(
  cd -- "$(dirname "$0")" >/dev/null 2>&1 || print_info "COULD NOT RETRIEVE ABSOLUTE PATH...MAKING ASSUMPTION THAT THE SUBPROJECTS DIRECTORY IS ./subprojects"
  pwd -P
)/subprojects"

print_info "CHECKING IF SUBPROJECTS DIRECTORY '$subprojects_directory' EXISTS..."
if [ ! -d "$subprojects_directory" ]; then
  print_info "SUBPROJECTS DIRECTORY DOES NOT EXIST"
  print_info "CREATING SUBPROJECTS DIRECTORY"
  mkdir "$subprojects_directory"

  if [ ! -d "$subprojects_directory" ]; then
    print_error "COULD NOT CREATE '$subprojects_directory' DIRECTORY...EXITING."
    exit 3
  fi

  print_good "SUBPROJECTS DIRECTORY CREATED"
else
  print_info "SUBPROJECTS DIRECTORY EXISTS"
fi

print_info "SETTING UP GTEST..."
meson wrap install gtest

print_info "CHECKING IF GTEST INSTALLED IN DIRECTORY"

directory_search="$subprojects_directory/*"

found="FALSE"
for FILE in $directory_search; do
  case $FILE in
  *"googletest"*)
      found="TRUE"
      print_good "GTEST FOUND"
      break
    ;;
  esac
done

if [ "$found" = "FALSE" ]; then
  print_error "GTEST NOT FOUND"
  print_error "ABORTING..."
  exit 4
fi

print_info "INVOKING MESON CONFIGURATION"
if meson setup buildDir --reconfigure; then
  print_good "ALL SETUP"
  print_good "HAPPY CODING!"
  exit 0
fi

print_error "SOMETHING FAILED :("
print_warning "MAYBE THE LOGS COULD PROVE USEFUL :)"
exit 20
