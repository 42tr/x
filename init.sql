CREATE TABLE `news` (
    `id` int unsigned NOT NULL,
    `content` text NOT NULL COMMENT '内容',
    `timestamp` bigint NOT NULL COMMENT '时间戳',
    `target` varchar(128) NOT NULL COMMENT '链接',
    PRIMARY KEY (`id`),
    KEY `time` (`timestamp`) USING BTREE
) ENGINE = InnoDB;

create table gold_info (
    timestamp bigint not null comment '时间戳',
    price float not null comment '价格',
    primary key (timestamp)
);

create table stock_info (
    timestamp bigint not null comment '时间戳',
    price float not null comment '价格',
    primary key (timestamp)
);
