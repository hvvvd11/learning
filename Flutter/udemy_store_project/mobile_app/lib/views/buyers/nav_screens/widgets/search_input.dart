import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';

class SearchInputWidget extends StatelessWidget {
  const SearchInputWidget({
    super.key,
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: EdgeInsets.only(left: 14, right: 15),
      child: ClipRRect(
        borderRadius: BorderRadius.circular(15),
        child: TextField(
            decoration: InputDecoration(
                fillColor: Colors.white24,
                filled: true,
                hintText: 'Search for a product',
                border: const OutlineInputBorder(borderSide: BorderSide.none),
                prefixIcon: Padding(
                  padding: const EdgeInsets.all(10),
                  child: SvgPicture.asset("assets/icons/search.svg"),
                ))),
      ),
    );
  }
}
