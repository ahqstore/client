APP_DEB="@appdeb"
APP_RPM="@apprpm"

ESCLATOR="@esc"

sudo_exists() {
  command -v sudo >/dev/null 2>&1
}

su_exists() {
  command -v su >/dev/null 2>&1
}

execute() {
  local cmd="$@"
  uid="$(id -u)"

  if [ "$uid" -eq "0" ]; then
    eval "$cmd"
  elif sudo_exists; then
    if sudo sh -c "$cmd"; then
      eval ""
    else
      echo "Error running sudo. Please run this script as root manually."
      exit 1
    fi
  elif su_exists; then
    if su -c "$cmd"; then
      eval ""
    else
      echo "Error running with 'su'. Please run this script as root manually."
      exit 1
    fi
  else
    echo "Neither 'sudo' nor 'su' found. Please run this script as root."
    exit 1
  fi
}

echo "░█████╗░██╗░░██╗░██████╗░  ░██████╗████████╗░█████╗░██████╗░███████╗
██╔══██╗██║░░██║██╔═══██╗  ██╔════╝╚══██╔══╝██╔══██╗██╔══██╗██╔════╝
███████║███████║██║██╗██║  ╚█████╗░░░░██║░░░██║░░██║██████╔╝█████╗░░
██╔══██║██╔══██║╚██████╔╝  ░╚═══██╗░░░██║░░░██║░░██║██╔══██╗██╔══╝░░
██║░░██║██║░░██║░╚═██╔═╝░  ██████╔╝░░░██║░░░╚█████╔╝██║░░██║███████╗
╚═╝░░╚═╝╚═╝░░╚═╝░░░╚═╝░░░  ╚═════╝░░░░╚═╝░░░░╚════╝░╚═╝░░╚═╝╚══════╝

Welcome to AHQ Store Installer!

Licensed under MIT. We are not responsible for any damages as outlined in out TOS:

In no event shall the developers or contributors be liable for any damages, including but not limited to direct, indirect, incidental, special, or consequential damages, arising out of the use or inability to use the software.

TOS: https://ahqstore.github.io/tos/index.html
Privacy Policy: https://ahqstore.github.io/privacy/index.html

None of the components are intentionally malicious, still you're advised to view the source before procceding:
https://github.com/orgs/ahqstore/repositories

You're advised to review the source code of every repository.

The developer, AHQ Softwares & AHQ Store Team has no liability
By continuing, you agree to the same"

echo "" # Add a blank line for readability between loops
echo "--- Installer Type ---"
echo "1) .deb"
echo "2) .rpm"
echo "----------------------"

TYPE=""

while true; do # Loop indefinitely until explicitly exited
  echo ""
  printf "Enter your choice: "
  read choice < /dev/tty

  case "$choice" in
    1)
      TYPE="deb"
      break
      ;;
    2)
      TYPE="rpm"
      break
      ;;
    *) # Invalid input
      echo "Invalid option: $choice. Please enter either 1 or 2."
      ;;
  esac
done

echo "
--------------------------------------------"
echo ""
echo "Introducing rootrunner for AHQ Store!

Rootrunner allows you to install AHQ Store Applications for all users
It is an SUID binary that escalates priviledges to install the \"3rd party app\".
Any user in the \`ahqstore\` group can run rootrunner without any permission requirement.

* Pros:
  - Allows you to install apps system-wide in linux
  - AHQ Store cannot install apps system-wide without rootrunner.

* Cons:
  - Rootrunner executes \"3rd party application\" installers as superuser posing significant risks.
  - This is designed to run third party apps & can run arbitrary code as root user.
  - Any application can execute rootrunner to escalate priviledges, provided the process is running
    as a user that has permission in the \`ahqstore\` group

There is a vulnerability in rootrunner due to the 3rd con as mentioned. Please be informed.

YOU CAN AVOID ALL THE AFORE MENTIONED RISKS BY NOT INSTALLING ROOTRUNNER

Do not install if you don't understand the meaning of the former paragraph.
We recommend not to install this."

printf "Would you like to install rootrunner? [y/N]"

while true; do # Loop indefinitely until explicitly exited
  read choice < /dev/tty

  case "$choice" in
    y|yes)
      TYPE="deb"
      break
      ;;
    n|no)
      TYPE="rpm"
      break
      ;;
    *) # Invalid input
      echo "Invalid option: $choice. Please enter either 1 or 2."
      ;;
  esac
done