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
        conn.request("GET", "/" + key)
        res = conn.getresponse()
        if res.status != 200:
            raise Exception("Wrong status - {0}".format(res.status))
        serv_val = res.read()
        if val.hexdigest() != serv_val:
            raise Exception("wrong response for {0}: {1} != {2}".format(key, val.hexdigest(), serv_val))
        count += 1
    print("{0} keys checked.".format(count))
