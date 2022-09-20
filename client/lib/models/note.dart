import 'package:notebook/models/user.dart';

class Note {
  final int id;
  final String title;
  final String content;
  final int authorId;
  final DateTime createdAt;
  final DateTime? updatedAt;
  final User? user;
  Note._(this.id, this.title, this.content, this.authorId, this.createdAt,
      this.updatedAt, this.user);

  factory Note.fromJson(Map<String, dynamic> json) {
    return Note._(
        json['id'],
        json['title'],
        json['content'],
        json['authorId'],
        DateTime.parse(json['createdAt']),
        json['updatedAt'] == null ? null : DateTime.parse(json['updatedAt']),
        json['user']);
  }
}
