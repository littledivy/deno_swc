#!/bin/sh
.\\call .\\tools\\lint.sh
.\\call .\\tools\\format.sh
.\\call .\\tools\\dev-test.sh
.\\call .\\tools\\e2e-test.sh
echo completed...
pause