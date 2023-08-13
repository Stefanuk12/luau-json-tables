-- [[ definition omitted ]] --

local Initial = {
    Color3 = Color3.new(1, 0, 0),
    Vector3 = Vector3.new(1, 0, 0),
    Vector2 = Vector2.new(1, 0),
    Inner = {
        Inner2 = {
            Color3 = Color3.new(1, 0, 0),
            Vector3 = Vector3.new(1, 0, 0),
            Vector2 = Vector2.new(1, 0),
        }
    }
}
local Formatted = TableEncode(Initial)
local FormattedJSON = HttpService:JSONEncode(Formatted)
warn(Formatted)

assert(HttpService:JSONDecode(FormattedJSON) == Formatted, "JSONDecode did not return the same table as TableEncode")
assert(Formatted == JSONDecode(Formatted), "JSONDecode did not return the same table as Initial")