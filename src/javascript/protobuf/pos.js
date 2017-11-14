'use strict'; // code generated by pbf v3.1.0

// RoomPosition ========================================

var RoomPosition = exports.RoomPosition = {};

RoomPosition.read = function (pbf, end) {
    return pbf.readFields(RoomPosition._readField, {x: 0, y: 0, roomName: ""}, end);
};
RoomPosition._readField = function (tag, obj, pbf) {
    if (tag === 1) obj.x = pbf.readVarint(true);
    else if (tag === 2) obj.y = pbf.readVarint(true);
    else if (tag === 3) obj.roomName = pbf.readString();
};
RoomPosition.write = function (obj, pbf) {
    if (obj.x) pbf.writeVarintField(1, obj.x);
    if (obj.y) pbf.writeVarintField(2, obj.y);
    if (obj.roomName) pbf.writeStringField(3, obj.roomName);
};
