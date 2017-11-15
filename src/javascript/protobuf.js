'use strict'; // code generated by pbf v3.1.0

var BodyPart = exports.BodyPart = {
    "WORK": 1,
    "MOVE": 2,
    "CARRY": 3,
    "ATTACK": 4,
    "RANGED_ATTACK": 5,
    "HEAL": 6,
    "TOUGH": 7,
    "CLAIM": 8
};

// RoomPosition ========================================

var RoomPosition = exports.RoomPosition = {};

RoomPosition.read = function (pbf, end) {
    return pbf.readFields(RoomPosition._readField, {x: 0, y: 0, roomName: ""}, end);
};
RoomPosition._readField = function (tag, obj, pbf) {
    if (tag === 1) obj.x = pbf.readVarint();
    else if (tag === 2) obj.y = pbf.readVarint();
    else if (tag === 3) obj.roomName = pbf.readString();
};
RoomPosition.write = function (obj, pbf) {
    if (obj.x) pbf.writeVarintField(1, obj.x);
    if (obj.y) pbf.writeVarintField(2, obj.y);
    if (obj.roomName) pbf.writeStringField(3, obj.roomName);
};

// StructureSpawn ========================================

var StructureSpawn = exports.StructureSpawn = {};

StructureSpawn.read = function (pbf, end) {
    return pbf.readFields(StructureSpawn._readField, {pos: null, hitsMax: 0, hits: 0, id: "", energy: 0, energyCapacity: 0, name: "", spawning: false}, end);
};
StructureSpawn._readField = function (tag, obj, pbf) {
    if (tag === 1) obj.pos = RoomPosition.read(pbf, pbf.readVarint() + pbf.pos);
    else if (tag === 2) obj.hitsMax = pbf.readVarint(true);
    else if (tag === 3) obj.hits = pbf.readVarint(true);
    else if (tag === 4) obj.id = pbf.readString();
    else if (tag === 5) obj.energy = pbf.readVarint(true);
    else if (tag === 6) obj.energyCapacity = pbf.readVarint(true);
    else if (tag === 7) obj.name = pbf.readString();
    else if (tag === 8) obj.spawning = pbf.readBoolean();
};
StructureSpawn.write = function (obj, pbf) {
    if (obj.pos) pbf.writeMessage(1, RoomPosition.write, obj.pos);
    if (obj.hitsMax) pbf.writeVarintField(2, obj.hitsMax);
    if (obj.hits) pbf.writeVarintField(3, obj.hits);
    if (obj.id) pbf.writeStringField(4, obj.id);
    if (obj.energy) pbf.writeVarintField(5, obj.energy);
    if (obj.energyCapacity) pbf.writeVarintField(6, obj.energyCapacity);
    if (obj.name) pbf.writeStringField(7, obj.name);
    if (obj.spawning) pbf.writeBooleanField(8, obj.spawning);
};

// RoomPosition ========================================

var RoomPosition = exports.RoomPosition = {};

RoomPosition.read = function (pbf, end) {
    return pbf.readFields(RoomPosition._readField, {x: 0, y: 0, roomName: ""}, end);
};
RoomPosition._readField = function (tag, obj, pbf) {
    if (tag === 1) obj.x = pbf.readVarint();
    else if (tag === 2) obj.y = pbf.readVarint();
    else if (tag === 3) obj.roomName = pbf.readString();
};
RoomPosition.write = function (obj, pbf) {
    if (obj.x) pbf.writeVarintField(1, obj.x);
    if (obj.y) pbf.writeVarintField(2, obj.y);
    if (obj.roomName) pbf.writeStringField(3, obj.roomName);
};

// CreepCarry ========================================

var CreepCarry = exports.CreepCarry = {};

CreepCarry.read = function (pbf, end) {
    return pbf.readFields(CreepCarry._readField, {resourceType: "", amount: 0}, end);
};
CreepCarry._readField = function (tag, obj, pbf) {
    if (tag === 1) obj.resourceType = pbf.readString();
    else if (tag === 2) obj.amount = pbf.readVarint(true);
};
CreepCarry.write = function (obj, pbf) {
    if (obj.resourceType) pbf.writeStringField(1, obj.resourceType);
    if (obj.amount) pbf.writeVarintField(2, obj.amount);
};

// Creep ========================================

var Creep = exports.Creep = {};

Creep.read = function (pbf, end) {
    return pbf.readFields(Creep._readField, {pos: null, carry: [], carryCapacity: 0, fatigue: 0, hits: 0, hitsMax: 0, id: "", my: false, name: "", spawning: false, ticksToLive: 0}, end);
};
Creep._readField = function (tag, obj, pbf) {
    if (tag === 1) obj.pos = RoomPosition.read(pbf, pbf.readVarint() + pbf.pos);
    else if (tag === 2) obj.carry.push(CreepCarry.read(pbf, pbf.readVarint() + pbf.pos));
    else if (tag === 3) obj.carryCapacity = pbf.readVarint(true);
    else if (tag === 4) obj.fatigue = pbf.readVarint(true);
    else if (tag === 5) obj.hits = pbf.readVarint(true);
    else if (tag === 6) obj.hitsMax = pbf.readVarint(true);
    else if (tag === 7) obj.id = pbf.readString();
    else if (tag === 8) obj.my = pbf.readBoolean();
    else if (tag === 9) obj.name = pbf.readString();
    else if (tag === 10) obj.spawning = pbf.readBoolean();
    else if (tag === 11) obj.ticksToLive = pbf.readVarint(true);
};
Creep.write = function (obj, pbf) {
    if (obj.pos) pbf.writeMessage(1, RoomPosition.write, obj.pos);
    if (obj.carry) for (var i = 0; i < obj.carry.length; i++) pbf.writeMessage(2, CreepCarry.write, obj.carry[i]);
    if (obj.carryCapacity) pbf.writeVarintField(3, obj.carryCapacity);
    if (obj.fatigue) pbf.writeVarintField(4, obj.fatigue);
    if (obj.hits) pbf.writeVarintField(5, obj.hits);
    if (obj.hitsMax) pbf.writeVarintField(6, obj.hitsMax);
    if (obj.id) pbf.writeStringField(7, obj.id);
    if (obj.my) pbf.writeBooleanField(8, obj.my);
    if (obj.name) pbf.writeStringField(9, obj.name);
    if (obj.spawning) pbf.writeBooleanField(10, obj.spawning);
    if (obj.ticksToLive) pbf.writeVarintField(11, obj.ticksToLive);
};

// RoomPosition ========================================

var RoomPosition = exports.RoomPosition = {};

RoomPosition.read = function (pbf, end) {
    return pbf.readFields(RoomPosition._readField, {x: 0, y: 0, roomName: ""}, end);
};
RoomPosition._readField = function (tag, obj, pbf) {
    if (tag === 1) obj.x = pbf.readVarint();
    else if (tag === 2) obj.y = pbf.readVarint();
    else if (tag === 3) obj.roomName = pbf.readString();
};
RoomPosition.write = function (obj, pbf) {
    if (obj.x) pbf.writeVarintField(1, obj.x);
    if (obj.y) pbf.writeVarintField(2, obj.y);
    if (obj.roomName) pbf.writeStringField(3, obj.roomName);
};

// XYPos ========================================

var XYPos = exports.XYPos = {};

XYPos.read = function (pbf, end) {
    return pbf.readFields(XYPos._readField, {x: 0, y: 0}, end);
};
XYPos._readField = function (tag, obj, pbf) {
    if (tag === 1) obj.x = pbf.readVarint();
    else if (tag === 2) obj.y = pbf.readVarint();
};
XYPos.write = function (obj, pbf) {
    if (obj.x) pbf.writeVarintField(1, obj.x);
    if (obj.y) pbf.writeVarintField(2, obj.y);
};

// Room ========================================

var Room = exports.Room = {};

Room.read = function (pbf, end) {
    return pbf.readFields(Room._readField, {name: "", energyAvailable: 0, energyCapacityAvailable: 0, sources: []}, end);
};
Room._readField = function (tag, obj, pbf) {
    if (tag === 1) obj.name = pbf.readString();
    else if (tag === 2) obj.energyAvailable = pbf.readVarint(true);
    else if (tag === 3) obj.energyCapacityAvailable = pbf.readVarint(true);
    else if (tag === 4) obj.sources.push(XYPos.read(pbf, pbf.readVarint() + pbf.pos));
};
Room.write = function (obj, pbf) {
    if (obj.name) pbf.writeStringField(1, obj.name);
    if (obj.energyAvailable) pbf.writeVarintField(2, obj.energyAvailable);
    if (obj.energyCapacityAvailable) pbf.writeVarintField(3, obj.energyCapacityAvailable);
    if (obj.sources) for (var i = 0; i < obj.sources.length; i++) pbf.writeMessage(4, XYPos.write, obj.sources[i]);
};

// World ========================================

var World = exports.World = {};

World.read = function (pbf, end) {
    return pbf.readFields(World._readField, {spawns: [], creeps: [], rooms: []}, end);
};
World._readField = function (tag, obj, pbf) {
    if (tag === 1) obj.spawns.push(StructureSpawn.read(pbf, pbf.readVarint() + pbf.pos));
    else if (tag === 2) obj.creeps.push(Creep.read(pbf, pbf.readVarint() + pbf.pos));
    else if (tag === 3) obj.rooms.push(Room.read(pbf, pbf.readVarint() + pbf.pos));
};
World.write = function (obj, pbf) {
    if (obj.spawns) for (var i = 0; i < obj.spawns.length; i++) pbf.writeMessage(1, StructureSpawn.write, obj.spawns[i]);
    if (obj.creeps) for (i = 0; i < obj.creeps.length; i++) pbf.writeMessage(2, Creep.write, obj.creeps[i]);
    if (obj.rooms) for (i = 0; i < obj.rooms.length; i++) pbf.writeMessage(3, Room.write, obj.rooms[i]);
};

// StructureSpawn ========================================

var StructureSpawn = exports.StructureSpawn = {};

StructureSpawn.read = function (pbf, end) {
    return pbf.readFields(StructureSpawn._readField, {pos: null, hitsMax: 0, hits: 0, id: "", energy: 0, energyCapacity: 0, name: "", spawning: false}, end);
};
StructureSpawn._readField = function (tag, obj, pbf) {
    if (tag === 1) obj.pos = RoomPosition.read(pbf, pbf.readVarint() + pbf.pos);
    else if (tag === 2) obj.hitsMax = pbf.readVarint(true);
    else if (tag === 3) obj.hits = pbf.readVarint(true);
    else if (tag === 4) obj.id = pbf.readString();
    else if (tag === 5) obj.energy = pbf.readVarint(true);
    else if (tag === 6) obj.energyCapacity = pbf.readVarint(true);
    else if (tag === 7) obj.name = pbf.readString();
    else if (tag === 8) obj.spawning = pbf.readBoolean();
};
StructureSpawn.write = function (obj, pbf) {
    if (obj.pos) pbf.writeMessage(1, RoomPosition.write, obj.pos);
    if (obj.hitsMax) pbf.writeVarintField(2, obj.hitsMax);
    if (obj.hits) pbf.writeVarintField(3, obj.hits);
    if (obj.id) pbf.writeStringField(4, obj.id);
    if (obj.energy) pbf.writeVarintField(5, obj.energy);
    if (obj.energyCapacity) pbf.writeVarintField(6, obj.energyCapacity);
    if (obj.name) pbf.writeStringField(7, obj.name);
    if (obj.spawning) pbf.writeBooleanField(8, obj.spawning);
};

// RoomPosition ========================================

var RoomPosition = exports.RoomPosition = {};

RoomPosition.read = function (pbf, end) {
    return pbf.readFields(RoomPosition._readField, {x: 0, y: 0, roomName: ""}, end);
};
RoomPosition._readField = function (tag, obj, pbf) {
    if (tag === 1) obj.x = pbf.readVarint();
    else if (tag === 2) obj.y = pbf.readVarint();
    else if (tag === 3) obj.roomName = pbf.readString();
};
RoomPosition.write = function (obj, pbf) {
    if (obj.x) pbf.writeVarintField(1, obj.x);
    if (obj.y) pbf.writeVarintField(2, obj.y);
    if (obj.roomName) pbf.writeStringField(3, obj.roomName);
};

// CreepCarry ========================================

var CreepCarry = exports.CreepCarry = {};

CreepCarry.read = function (pbf, end) {
    return pbf.readFields(CreepCarry._readField, {resourceType: "", amount: 0}, end);
};
CreepCarry._readField = function (tag, obj, pbf) {
    if (tag === 1) obj.resourceType = pbf.readString();
    else if (tag === 2) obj.amount = pbf.readVarint(true);
};
CreepCarry.write = function (obj, pbf) {
    if (obj.resourceType) pbf.writeStringField(1, obj.resourceType);
    if (obj.amount) pbf.writeVarintField(2, obj.amount);
};

// Creep ========================================

var Creep = exports.Creep = {};

Creep.read = function (pbf, end) {
    return pbf.readFields(Creep._readField, {pos: null, carry: [], carryCapacity: 0, fatigue: 0, hits: 0, hitsMax: 0, id: "", my: false, name: "", spawning: false, ticksToLive: 0}, end);
};
Creep._readField = function (tag, obj, pbf) {
    if (tag === 1) obj.pos = RoomPosition.read(pbf, pbf.readVarint() + pbf.pos);
    else if (tag === 2) obj.carry.push(CreepCarry.read(pbf, pbf.readVarint() + pbf.pos));
    else if (tag === 3) obj.carryCapacity = pbf.readVarint(true);
    else if (tag === 4) obj.fatigue = pbf.readVarint(true);
    else if (tag === 5) obj.hits = pbf.readVarint(true);
    else if (tag === 6) obj.hitsMax = pbf.readVarint(true);
    else if (tag === 7) obj.id = pbf.readString();
    else if (tag === 8) obj.my = pbf.readBoolean();
    else if (tag === 9) obj.name = pbf.readString();
    else if (tag === 10) obj.spawning = pbf.readBoolean();
    else if (tag === 11) obj.ticksToLive = pbf.readVarint(true);
};
Creep.write = function (obj, pbf) {
    if (obj.pos) pbf.writeMessage(1, RoomPosition.write, obj.pos);
    if (obj.carry) for (var i = 0; i < obj.carry.length; i++) pbf.writeMessage(2, CreepCarry.write, obj.carry[i]);
    if (obj.carryCapacity) pbf.writeVarintField(3, obj.carryCapacity);
    if (obj.fatigue) pbf.writeVarintField(4, obj.fatigue);
    if (obj.hits) pbf.writeVarintField(5, obj.hits);
    if (obj.hitsMax) pbf.writeVarintField(6, obj.hitsMax);
    if (obj.id) pbf.writeStringField(7, obj.id);
    if (obj.my) pbf.writeBooleanField(8, obj.my);
    if (obj.name) pbf.writeStringField(9, obj.name);
    if (obj.spawning) pbf.writeBooleanField(10, obj.spawning);
    if (obj.ticksToLive) pbf.writeVarintField(11, obj.ticksToLive);
};

// RoomPosition ========================================

var RoomPosition = exports.RoomPosition = {};

RoomPosition.read = function (pbf, end) {
    return pbf.readFields(RoomPosition._readField, {x: 0, y: 0, roomName: ""}, end);
};
RoomPosition._readField = function (tag, obj, pbf) {
    if (tag === 1) obj.x = pbf.readVarint();
    else if (tag === 2) obj.y = pbf.readVarint();
    else if (tag === 3) obj.roomName = pbf.readString();
};
RoomPosition.write = function (obj, pbf) {
    if (obj.x) pbf.writeVarintField(1, obj.x);
    if (obj.y) pbf.writeVarintField(2, obj.y);
    if (obj.roomName) pbf.writeStringField(3, obj.roomName);
};

// XYPos ========================================

var XYPos = exports.XYPos = {};

XYPos.read = function (pbf, end) {
    return pbf.readFields(XYPos._readField, {x: 0, y: 0}, end);
};
XYPos._readField = function (tag, obj, pbf) {
    if (tag === 1) obj.x = pbf.readVarint();
    else if (tag === 2) obj.y = pbf.readVarint();
};
XYPos.write = function (obj, pbf) {
    if (obj.x) pbf.writeVarintField(1, obj.x);
    if (obj.y) pbf.writeVarintField(2, obj.y);
};

// Room ========================================

var Room = exports.Room = {};

Room.read = function (pbf, end) {
    return pbf.readFields(Room._readField, {name: "", energyAvailable: 0, energyCapacityAvailable: 0, sources: []}, end);
};
Room._readField = function (tag, obj, pbf) {
    if (tag === 1) obj.name = pbf.readString();
    else if (tag === 2) obj.energyAvailable = pbf.readVarint(true);
    else if (tag === 3) obj.energyCapacityAvailable = pbf.readVarint(true);
    else if (tag === 4) obj.sources.push(XYPos.read(pbf, pbf.readVarint() + pbf.pos));
};
Room.write = function (obj, pbf) {
    if (obj.name) pbf.writeStringField(1, obj.name);
    if (obj.energyAvailable) pbf.writeVarintField(2, obj.energyAvailable);
    if (obj.energyCapacityAvailable) pbf.writeVarintField(3, obj.energyCapacityAvailable);
    if (obj.sources) for (var i = 0; i < obj.sources.length; i++) pbf.writeMessage(4, XYPos.write, obj.sources[i]);
};

// CreepSpawn ========================================

var CreepSpawn = exports.CreepSpawn = {};

CreepSpawn.read = function (pbf, end) {
    return pbf.readFields(CreepSpawn._readField, {spawn_name: "", body: [], creep_name: ""}, end);
};
CreepSpawn._readField = function (tag, obj, pbf) {
    if (tag === 1) obj.spawn_name = pbf.readString();
    else if (tag === 2) pbf.readPackedVarint(obj.body);
    else if (tag === 3) obj.creep_name = pbf.readString();
};
CreepSpawn.write = function (obj, pbf) {
    if (obj.spawn_name) pbf.writeStringField(1, obj.spawn_name);
    if (obj.body) pbf.writePackedVarint(2, obj.body);
    if (obj.creep_name) pbf.writeStringField(3, obj.creep_name);
};
