// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'note.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Note _$NoteFromJson(Map<String, dynamic> json) => Note(
      json['id'] as int,
      json['title'] as String,
      json['content'] as String,
      json['authorId'] as int,
      DateTime.parse(json['createdAt'] as String),
      json['updatedAt'] == null
          ? null
          : DateTime.parse(json['updatedAt'] as String),
      json['user'] == null
          ? null
          : User.fromJson(json['user'] as Map<String, dynamic>),
    );

Map<String, dynamic> _$NoteToJson(Note instance) => <String, dynamic>{
      'id': instance.id,
      'title': instance.title,
      'content': instance.content,
      'authorId': instance.authorId,
      'createdAt': instance.createdAt.toIso8601String(),
      'updatedAt': instance.updatedAt?.toIso8601String(),
      'user': instance.user?.toJson(),
    };

NoteConnection _$NoteConnectionFromJson(Map<String, dynamic> json) =>
    NoteConnection(
      PageInfo.fromJson(json['pageInfo'] as Map<String, dynamic>),
      (json['edges'] as List<dynamic>)
          .map((e) => NoteEdge.fromJson(e as Map<String, dynamic>))
          .toList(),
    );

Map<String, dynamic> _$NoteConnectionToJson(NoteConnection instance) =>
    <String, dynamic>{
      'pageInfo': instance.pageInfo.toJson(),
      'edges': instance.edges.map((e) => e.toJson()).toList(),
    };

NoteEdge _$NoteEdgeFromJson(Map<String, dynamic> json) => NoteEdge(
      json['cursor'] as String,
      Note.fromJson(json['node'] as Map<String, dynamic>),
    );

Map<String, dynamic> _$NoteEdgeToJson(NoteEdge instance) => <String, dynamic>{
      'cursor': instance.cursor,
      'node': instance.node.toJson(),
    };
