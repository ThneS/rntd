/*
 Navicat Premium Data Transfer

 Source Server         : ntd
 Source Server Type    : PostgreSQL
 Source Server Version : 160003 (160003)
 Source Host           : localhost:15432
 Source Catalog        : ntd
 Source Schema         : public

 Target Server Type    : PostgreSQL
 Target Server Version : 160003 (160003)
 File Encoding         : 65001

 Date: 01/06/2024 22:20:52
*/


-- ----------------------------
-- Table structure for event
-- ----------------------------
DROP TABLE IF EXISTS "public"."event";
CREATE TABLE "public"."event" (
  "id" int8 NOT NULL GENERATED ALWAYS AS IDENTITY (
INCREMENT 1
MINVALUE  1
MAXVALUE 9223372036854775807
START 1
CACHE 1
),
  "task_id" int8 NOT NULL,
  "alarm_id" int8 NOT NULL,
  "create_time" date NOT NULL,
  "timestamp" timestamp(6) NOT NULL,
  "advice" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "level" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "module" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "sub_module" varchar(255) COLLATE "pg_catalog"."default" NOT NULL
)
;
ALTER TABLE "public"."event" OWNER TO "ntd";

-- ----------------------------
-- Primary Key structure for table event
-- ----------------------------
ALTER TABLE "public"."event" ADD CONSTRAINT "event_pkey" PRIMARY KEY ("id");
