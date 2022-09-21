import 'package:json_annotation/json_annotation.dart';

part 'user.g.dart';

@JsonSerializable()
class User {
  final int id;
  final String name;
  final String? email;
  final String? avatar;
  final DateTime createdAt;
  final DateTime? updatedAt;
  User(this.id, this.name, this.email, this.avatar, this.createdAt,
      this.updatedAt);

  factory User.fromJson(Map<String, dynamic> json) => _$UserFromJson(json);
  Map<String, dynamic> toJson() => _$UserToJson(this);
}
