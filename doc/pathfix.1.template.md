% PATHFIX(1) Version ${VERSION} | Fixes the $PATH mess

NAME
====

**pathfix** - Fixes the $PATH mess.

SYNOPSIS
========

| **pathfix** \[**OPTIONS**]

DESCRIPTION
===========

**pathfix** generates the *PATH* environment variable
by including pre configured paths if the exist.
Because **pathfix** comes with batteries included,
the user normaly does not have to add additional configuration.
In the default configuration existing paths in *PATH* will be
included, but will be deduplicated if they exist multiple times.

To use pathfix add this to the end of *~/.bashrc*, *~/.zshrc*, ...

```
# ~/.bashrc 
export PATH=$(/usr/bin/pathfix)
```

Options
-------

-d, --dedup       

: Deduplicates the path

-D, --defaults    

: Use this flag to use the recommended settings for pathfix.
  Usually you don't need another configuration and adding
  'export PATH=$(/usr/bin/pathfix -D)' to your .bashrc/.zshrc/... file is enough. 
    
-e, --from-env    

: Includes path's from $PATH in environment

-h, --help        

: Prints help information

-i, --included    

: Searches included path's using inbuild configuration

-l, --lines       

: Outputs line by line instead of the default colon seperated list

-V, --version     

: Prints version information

FILES
=====

*~/.pathfix.toml*

: Per-user configuration file

*/etc/pathfix.toml*

: Global configuration file

BUGS
====

See GitHub Issues: <https://github.com/rappet/pathfix/issues>

ENVIRONMENT
===========

**PATH**

: Contents will be added to generated path variable if _-D_ or _-e_ is set.

AUTHOR
======

Raphael Peters <raphael.r.peters@gmail.com>

SEE ALSO
========

**pathfix.toml(5)**
