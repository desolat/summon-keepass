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

`summon-keepass` uses the following syntax for secrets:
  
    [group/subgroup/]entry[|field]

By default the `Password` field value of the `entry` will be returned. If an alternative `field` name is being provided, the value of that field will be returned.

Here's an example of a `secrets.yml` file:

    AWS_ACCESS_KEY_ID: !var aws/iam/user/robot/access_key_id
    AWS_SECRET_ACCESS_KEY: !var aws/iam/user/robot/secret_access_key
    SSH_PRIV_KEY: !var:file ssh/some server|priv_key

Todo
----
- tests
- get the KeePass DB password from an environment variable (preferred)
- key file authentication
- error handling for incorrect config/KeePass DB file path
