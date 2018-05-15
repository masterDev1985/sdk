#!/bin/sh

source ./shared.functions.sh

START_DIR=$PWD
WORK_DIR=$START_DIR/../../../../../.macosbuild
mkdir -p $WORK_DIR
WORK_DIR=$(abspath "$WORK_DIR")

mkdir -p $WORK_DIR/libzmq-android
cp -rp ../libsodium $WORK_DIR/libzmq-android/
cp -rp ../zmq $WORK_DIR/libzmq-android/

cd $WORK_DIR/libzmq-android/libsodium
chmod a+x build.sh
./build.sh arm 21 arm-linux-androideabi
##docker run -it -v /Users/norm/forge/work/code/evernym/sdk-evernym/.macosbuild/libzmq-android:/data sodium-android:latest
cp $START_DIR/linux.build.more.android.architectures.sh $WORK_DIR/libzmq-android
docker run -v $WORK_DIR/libzmq-android:/data --rm --entrypoint /data/linux.build.more.android.architectures.sh sodium-android
##cp linux.build.more.android.architectures.sh ../../../../../.macosbuild/libzmq-android
##docker run -v /Users/norm/forge/work/code/evernym/sdk-evernym/.macosbuild/libzmq-android:/data --rm --entrypoint /data/linux.build.more.android.architectures.sh sodium-android
