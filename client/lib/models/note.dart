import 'package:json_annotation/json_annotation.dart';
import 'package:notebook/models/connection.dart';
import 'package:notebook/models/user.dart';

part 'note.g.dart';

@JsonSerializable(explicitToJson: true)
class Note {
  final int id;
  final String title;
  final String content;
  final int authorId;
  final DateTime createdAt;
  final DateTime? updatedAt;
  final User? user;
  Note(this.id, this.title, this.content, this.authorId, this.createdAt,
      this.updatedAt, this.user);

  factory Note.fromJson(Map<String, dynamic> json) => _$NoteFromJson(json);
  Map<String, dynamic> toJson() => _$NoteToJson(this);
}

@JsonSerializable(explicitToJson: true)
class NoteConnection extends Connection<NoteEdge> {
  NoteConnection(PageInfo pageInfo, List<NoteEdge> edges)
      : super(pageInfo, edges);

  factory NoteConnection.fromJson(Map<String, dynamic> json) =>
      _$NoteConnectionFromJson(json);
  Map<String, dynamic> toJson() => _$NoteConnectionToJson(this);
}

@JsonSerializable(explicitToJson: true)
class NoteEdge extends Edge<Note> {
  NoteEdge(String cursor, Note node) : super(cursor, node);

  factory NoteEdge.fromJson(Map<String, dynamic> json) =>
      _$NoteEdgeFromJson(json);
  Map<String, dynamic> toJson() => _$NoteEdgeToJson(this);
}
