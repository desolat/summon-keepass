summon-keepass
==============

Simple [summon](https://cyberark.github.io/summon/) provider that allows usage of KeePass kdbx database files.

Installation
-----

Download the archive for your OS from the [latest release](../../releases), extract it and provide the executable to summon [as a provider](https://cyberark.github.io/summon/#providers). 

Alternatively there is an [install script](./install.sh) available.

Create a `.summon-keepass.ini` file in your `$HOME` directory with the following content:

    [keepass_db]
    path=/path/to/your/keepass_database_file.kdbx
    pass=password to your keepass database

Usage
-----

Let's say you have the following entries in your `secrets.yml` file:

    AWS_ACCESS_KEY_ID: !var aws/iam/user/robot/access_key_id
    AWS_SECRET_ACCESS_KEY: !var aws/iam/user/robot/secret_access_key

`summon-keepass` will split each secret with `/` and then return the password from the database entry whose title matches the last part of the secret and is placed in correct group determined by previous parts of the secret.

In that case KeePass database should look like this:
![KeePass example](https://imgur.com/SPdha3h.png)

Todo
----
- tests
- get the KeePass DB password from an environment variable (preferred)
- error handling for incorrect config/KeePass DB file path
