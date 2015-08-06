#!/usr/bin/env python
# -*- coding: utf-8 -*-

import httplib2
import sys

if __name__ == '__main__':
    for line in sys.stdin:
        conn = httplib2.HTTPConnectionWithTimeout("localhost", int(sys.argv[1]))
        conn.request("POST", "/"+line.rstrip(), "STUFF")
        res = conn.getresponse()
