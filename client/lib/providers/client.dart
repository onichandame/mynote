import 'package:flutter/foundation.dart';
import 'package:graphql/client.dart';
import 'package:notebook/models/user.dart';
import 'package:notebook/providers/api_schema.dart';
import 'package:shared_preferences/shared_preferences.dart';

class Client extends ChangeNotifier {
  final SharedPreferences? sharedPrefs;
  final ApiSchema? apiSchema;

  static const String _sessionKey = "auth";
  static const url =
      String.fromEnvironment('API_URL', defaultValue: 'http://localhost');
  String? _session;
  late GraphQLClient _client;
  User? _user;
  bool _loading = false;

  Client(this.sharedPrefs, this.apiSchema) {
    _session = sharedPrefs?.getString(_sessionKey);
    _client = _getClient();
  }

  GraphQLClient _getClient() {
    final httpLink = HttpLink(url);
    final wsLink = WebSocketLink(url);
    final transportLink =
        Link.split((request) => request.isSubscription, wsLink, httpLink);
    final authLink =
        AuthLink(getToken: () => _session == null ? null : 'Bearer $_session');
    final link = authLink.concat(transportLink);
    return GraphQLClient(link: link, cache: GraphQLCache());
  }

  GraphQLClient get client => _client;

  String? get session => _session;

  User? get user => _user;

  bool get loading => _loading;

  set session(String? sess) {
    bool changed = sess == _session;
    _session = sess;
    if (changed) {
      _client = _getClient();
      if (session != null) {
        _loading = true;
        getUser().then((v) {
          _user = v;
        }).whenComplete(() {
          _loading = false;
          notifyListeners();
        });
      }
      notifyListeners();
    }
  }

  Future<User?> getUser() async {
    if (session == null) return null;
    return User.fromJson((await _request(operationName: 'users'))?['edges'][0]);
  }

  String get _schema {
    return apiSchema?.schema ?? '';
  }

  Future<dynamic> _request(
      {required String operationName, String? resultName}) async {
    final res = await client.query(
        QueryOptions(document: gql(_schema), operationName: operationName));
    final error = res.exception;
    if (error != null) throw error;
    final rn = resultName ?? operationName;
    final data = res.data?[rn];
    if (data == null) throw Exception('$rn not found');
    return data;
  }
}
