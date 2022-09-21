// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'connection.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

PageInfo _$PageInfoFromJson(Map<String, dynamic> json) => PageInfo(
      json['hasPreviousPage'] as bool,
      json['hasNextPage'] as bool,
      json['startCursor'] as String,
      json['endCursor'] as String,
    );

Map<String, dynamic> _$PageInfoToJson(PageInfo instance) => <String, dynamic>{
      'hasPreviousPage': instance.hasPreviousPage,
      'hasNextPage': instance.hasNextPage,
      'startCursor': instance.startCursor,
      'endCursor': instance.endCursor,
    };
