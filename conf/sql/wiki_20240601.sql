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

 Date: 01/06/2024 22:21:12
*/


-- ----------------------------
-- Table structure for wiki
-- ----------------------------
DROP TABLE IF EXISTS "public"."wiki";
CREATE TABLE "public"."wiki" (
  "alarm_id" int8 NOT NULL,
  "wiki" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "advice" varchar(255) COLLATE "pg_catalog"."default",
  "level" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "module" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "sub_module" varchar(255) COLLATE "pg_catalog"."default" NOT NULL,
  "create_time" date NOT NULL,
  "update_time" date,
  "enable" bool NOT NULL
)
;
ALTER TABLE "public"."wiki" OWNER TO "ntd";

-- ----------------------------
-- Primary Key structure for table wiki
-- ----------------------------
ALTER TABLE "public"."wiki" ADD CONSTRAINT "wiki_pkey" PRIMARY KEY ("alarm_id");
