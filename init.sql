CREATE TABLE `news` (
  `id` int unsigned NOT NULL,
  `content` text NOT NULL COMMENT '内容',
  `timestamp` bigint NOT NULL COMMENT '时间戳',
  `target` varchar(128) NOT NULL COMMENT '链接',
  PRIMARY KEY (`id`),
  KEY `time` (`timestamp`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
