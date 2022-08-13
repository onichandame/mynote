import 'package:flutter/services.dart';

class ApiSchema {
  late String _schema;

  ApiSchema._(this._schema);

  static Future<ApiSchema> create() async {
    return ApiSchema._(await rootBundle.loadString('assets/api.graphql'));
  }

  String get schema {
    return _schema;
  }
}
