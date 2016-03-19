#!/usr/bin/env python
# -*- coding: utf-8 -*-

import httplib2
import sys
import md5


if __name__ == '__main__':
    count = 0
    for line in sys.stdin:
        conn = httplib2.HTTPConnectionWithTimeout("localhost", int(sys.argv[1]))
        key = line.rstrip()
        val = md5.md5(key)
        conn.request("POST", "/" + key, val.hexdigest())
        res = conn.getresponse()
        if res.status != 200:
            raise Exception("Wrong status - {0}".format(res.status))
        count += 1
    print("{0} keys written.".format(count))
