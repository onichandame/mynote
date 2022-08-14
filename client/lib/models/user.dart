class User {
  final int id;
  final String name;
  final String? email;
  final String? avatar;
  final DateTime createdAt;
  final DateTime? updatedAt;
  User._(this.id, this.name, this.email, this.avatar, this.createdAt,
      this.updatedAt);

  factory User.fromJson(Map<String, dynamic> json) {
    return User._(
        json['id'],
        json['name'],
        json['email'],
        json['avatar'],
        DateTime.parse(json['createdAt']),
        json['updatedAt'] == null ? null : DateTime.parse(json['updatedAt']));
  }
}
